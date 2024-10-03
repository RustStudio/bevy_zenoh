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
        .add_systems(Startup, query_server)
        .add_systems(FixedUpdate, generate_value)
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}

fn query_server(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let key_expr: KeyExpr<'static> = KeyExpr::new("key/expression").unwrap();
        let session = zenoh::open(config::default()).res().await.unwrap();
        let queryable = session
            .declare_queryable(key_expr.clone())
            .res()
            .await
            .unwrap();

        while let Ok(query) = queryable.recv_async().await {
            info!(">> Received {:?}", query.value().unwrap().to_string());
            let value = ctx
                .run_on_main_thread(move |ctx| match ctx.world.get_resource::<MyResource>() {
                    Some(my_resource) => {
                        return my_resource.0.clone();
                    }
                    None => {
                        panic!("Resource not found");
                    }
                })
                .await;
            let reply = Ok(Sample::new(key_expr.clone(), value));
            query.reply(reply).res().await.unwrap();
        }
    });
}

fn generate_value(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    my_resource.0 = format!("The time is {:?}", time.elapsed_seconds().round());
}
