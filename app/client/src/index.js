const ws = new WebSocket('ws://localhost:8080');

ws.addEventListener('open', () => {
  ws.send('Hello from client side');
});

ws.addEventListener('message', event => {
  console.log('Got:', event.data);
});
