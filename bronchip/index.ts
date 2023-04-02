document.getElementById("request-serial").onclick = () => {
  stream();
};

const stream = async () => {
  const port = await navigator.serial.requestPort();
  await port.open({ baudRate: 115200 });

  const rust = await import("./pkg");
  let src: UnderlyingDefaultSource = new rust.StreamSource(port.readable);

  const rs = new ReadableStream(src);
  const rad = rs.getReader();
  while (true) {
    let { value, done } = await rad.read();
    if (done) {
      break;
    }
    let thing = document.getElementById("id01") as HTMLInputElement;
    thing.value += value;
    thing.scrollTop = thing.scrollHeight;
  }
};
