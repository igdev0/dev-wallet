import Screen from "../components/screen.tsx";
import { Heading, Text } from "@chakra-ui/react";
import useListAccounts from "../hooks/use-list-accounts.ts";

export default function AccountsScreen() {
  let accountsRes = useListAccounts();

  return (
    <Screen>
      <Heading>Accounts screen</Heading>
      {accountsRes.data?.map((account, index) => (
        <div key={index}>
          <Text>{account.address}</Text>
          <Text>{account.network}</Text>
        </div>
      ))}
    </Screen>
  );
}