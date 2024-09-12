import Screen from "../components/screen.tsx";
import { Heading, Text } from "@chakra-ui/react";
import Error from "../components/error.tsx";
import useListAccounts from "../hooks/use-list-accounts.ts";
import Loading from "../components/loading.tsx";

export default function AccountsScreen() {
  let accountsRes = useListAccounts();
  if (accountsRes.isError) {
    return (
      <Error error="This is not what you're looking for. Please login to list your accounts." />
    );
  }
  if (accountsRes.isLoading) {
    return <Loading />;
  }

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