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
use zenoh::prelude::r#async::*;

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_systems(Startup, subscriber)
        .run();
}

fn subscriber(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let session = zenoh::open(config::default()).res().await.unwrap();
        let subscriber = session.declare_subscriber("zb_publisher/**").res().await.unwrap();
        while let Ok(sample) = subscriber.recv_async().await {
            println!("Key expr: {} - Received: {}", sample.key_expr, sample);
        };
    });
}
