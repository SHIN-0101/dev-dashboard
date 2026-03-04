use dev_dashboard::data::fetcher::DataMessage;

#[tokio::test]
async fn test_data_message_channel() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<DataMessage>(32);
    tx.send(DataMessage::GitUpdated(None)).await.unwrap();
    let msg = rx.recv().await.unwrap();
    assert!(matches!(msg, DataMessage::GitUpdated(_)));
}
