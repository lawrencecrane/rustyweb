const ws = new WebSocket('ws://localhost:8080/ws', 'json');

ws.addEventListener('open', () => {
  const data = { msg: "Hello from the client side" };

  ws.send(JSON.stringify(data));
});

ws.addEventListener('message', event => {
  console.log('Got:', JSON.parse(event.data));
});

ws.addEventListener('error', event => {
  console.log(event);
});

ws.addEventListener('close', () => {
  console.log('Connection closed');
});
