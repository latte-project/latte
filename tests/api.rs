use latte::{client::connect, func::LatteObject, Result};

#[tokio::main]
#[test]
async fn setget_test() -> Result<()> {
    let mut client = connect("127.0.0.1:6379").await?;
    let object_ref = String::from("my_id");
    let object = LatteObject::String(String::from("This is the value"));
    client.set(&object_ref, object.clone()).await?;
    let received = client.get(&object_ref).await?;
    assert_eq!(object, received);
    Ok(())
}

#[tokio::main]
#[test]
async fn set_modify_test() -> Result<()> {
    let mut client = connect("127.0.0.1:6379").await?;
    let object_ref = String::from("my_id1");
    let object = LatteObject::Float(3.14);
    client.set(&object_ref, object.clone()).await?;

    let new_object = LatteObject::Integer(114514);
    client.set(&object_ref, new_object.clone()).await?;

    let received = client.get(&object_ref).await?;
    assert_eq!(new_object, received);

    Ok(())
}