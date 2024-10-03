![Cargo build](https://github.com/RustStudio/bevy_zenoh/actions/workflows/build.yaml/badge.svg) ![Cargo test](https://github.com/RustStudio/bevy_zenoh/actions/workflows/test.yaml/badge.svg) ![Cargo format](https://github.com/RustStudio/bevy_zenoh/actions/workflows/format.yaml/badge.svg)

# Bevy with Bevy
This repository is to provide simple bevy-zenoh integration

# Examples

## zb_publisher

In this example, we demostrate the ability to create 2 different kind of publisher.

The first kind is a bevy publisher where the rate of publishing is regulated by bevy's updating rate. This method allows easy access of bevy's resources and integration with other bevy features like queries.

The second kind is a tokio publisher where the rate of publishing is regulated by a tokio background node. This method frees the publisher from the bevy's updating rate, allowing it to publish at a faster or slower rate.

## zb_subscriber

In this example, we created a tokio background subscriber that listens to a topic. This method is recommended as a subscriber blocks the thread with a `while` checker, which will cause the bevy application to hang if implemented on the main thread.

## zb_query_server

In this example, we created a tokio background query server. The server will listen on the query topic and execute a reply for which the server will access information from the main thread.

## zb_query_client

In this example, we created a tokio background query client. The client will attempt to get a reply from the server and store the reply in the global resource.