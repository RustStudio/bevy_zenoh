// ==========================================================================
/*
 * Copyright (C) 2024 Rust Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// ==========================================================================
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use bevy_zenoh::ZenohResource;
use zenoh::prelude::r#async::*;

#[derive(Default, Resource)]
struct MyResource(i64, i64);

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .init_resource::<MyResource>()
        .add_systems(Startup, background_publisher)
        .add_systems(FixedUpdate, bevy_publisher)
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}

fn background_publisher(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let session = zenoh::open(config::default()).res().await.unwrap();

        loop {
            let value = ctx
                .run_on_main_thread(move |ctx| match ctx.world.get_resource::<MyResource>() {
                    Some(my_resource) => {
                        return my_resource.1.clone();
                    }
                    None => {
                        panic!("Resource not found");
                    }
                })
                .await;

            session
                .put("zb_publisher/background_publisher", value)
                .res()
                .await
                .unwrap();

            ctx.run_on_main_thread(
                move |ctx| match ctx.world.get_resource_mut::<MyResource>() {
                    Some(mut my_resource) => {
                        return my_resource.1 += 1;
                    }
                    None => {
                        panic!("Resource not found");
                    }
                },
            )
            .await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
}

#[tokio::main]
async fn bevy_publisher(zenoh_resource: Local<ZenohResource>, mut my_resource: ResMut<MyResource>) {
    zenoh_resource
        .session
        .put("zb_publisher/bevy_publisher", my_resource.0)
        .res()
        .await
        .unwrap();
    my_resource.0 += 1;
}
