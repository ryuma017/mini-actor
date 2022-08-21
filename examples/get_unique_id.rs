use mini_actor::ActorHandle;

async fn get_id_then_print(handle: ActorHandle) {
    let id = handle.get_unique_id().await;
    println!("{id}");
}

#[tokio::main]
async fn main() {
    let handle = ActorHandle::new();
    tokio::join!(
        get_id_then_print(handle.clone()),
        get_id_then_print(handle.clone()),
        get_id_then_print(handle.clone()),
    );
}
