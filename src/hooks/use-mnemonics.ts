import { useQuery } from "react-query";
import { invoke } from "@tauri-apps/api";

export default function useMnemonics() {
  const query = useQuery<unknown, string, string>(
    "mnemonics-key",
    async () => {
      const res: string[] = await invoke("generate_mnemonic");
      return res;
    },
    {
      refetchOnMount: true,
      cacheTime: 0,
      structuralSharing: false,
      onSuccess: (data) => {
        // return data.split(" ");
        console.log(data.split(" ").length);
      },
    },
  );
  return query;
}