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
use core::panic;
use zenoh::prelude::r#async::*;

#[derive(Default, Resource)]
struct MyResource(String);

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .init_resource::<MyResource>()
        .add_systems(Startup, query_client)
        .add_systems(FixedUpdate, print_value)
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}

fn query_client(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let session = zenoh::open(config::default()).res().await.unwrap();

        let value = "What time is it now?";
        info!("Sending: {:?}", value);
        let replies = session
            .get("key/expression")
            .with_value(value)
            .res()
            .await
            .unwrap();

        while let Ok(reply) = replies.recv_async().await {
            ctx.run_on_main_thread(
                move |ctx| match ctx.world.get_resource_mut::<MyResource>() {
                    Some(mut my_resource) => {
                        my_resource.0 = reply.sample.unwrap().to_string();
                    }
                    None => {
                        panic!("Resource not found");
                    }
                },
            )
            .await;
        }
    });
}

fn print_value(my_resource: Res<MyResource>) {
    info!("Resource: {:?}", my_resource.0);
}
