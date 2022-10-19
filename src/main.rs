#[tokio::main}

async fn main() -> io::Result<()>{
  let db: Arc<(Mutex<HashMap<String, ValueStore>>> = Arc::new(Mutex::new(HashMap::new())));
  let listener = TcpListener::bid("127.0.0.1.8080").await?;
  
  loop{
  let(mut socket = listener.accept().await?;
  let db = db.come();
