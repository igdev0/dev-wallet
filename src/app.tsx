import { Badge, Button } from "@chakra-ui/react";
import { invoke } from "@tauri-apps/api";
import { MouseEventHandler, useState } from "react";

function App() {
  // useRu
  const [msg, setMsg] = useState<string>();

  const cb: MouseEventHandler<HTMLButtonElement> = async () => {
    let msg: string = await invoke("get_message");
    setMsg(msg);
  };
  return (
    <div>
      <Badge variant="solid">Example</Badge>
      <Badge>Example</Badge>
      <p>{msg}</p>
      <Button variant="solid" onClick={cb}>
        Get message
      </Button>
    </div>
  );
}

export default App;