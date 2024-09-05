import Screen from "../components/screen.tsx";
import { Heading } from "@chakra-ui/react";
import { useParams } from "react-router-dom";
import Error from "../components/error.tsx";

export default function AccountsScreen() {
  const params = useParams<{ wallet_id: string }>();

  if (!params.wallet_id) {
    return (
      <Error error="This is not what you're looking for. Please login to list your accounts." />
    );
  }

  return (
    <Screen>
      <Heading>Accounts screen</Heading>
    </Screen>
  );
}