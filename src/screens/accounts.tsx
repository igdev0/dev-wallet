import Screen from "../components/screen.tsx";
import {
  Badge,
  IconButton,
  Table,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import useListAccounts from "../hooks/use-list-accounts.ts";
import Navbar from "../components/navbar.tsx";
import { Copy01, Trash01 } from "@untitled-ui/icons-react";

export default function AccountsScreen() {
  const accountsRes = useListAccounts();

  return (
    <Screen>
      <Navbar />
      <Table overflowY="auto">
        <Thead>
          <Th>Address</Th>
          <Th>Blockchain</Th>
          <Th>Network</Th>
        </Thead>
        <Tbody>
          {accountsRes.data?.map((item, index) => (
            <Tr key={index}>
              <Td>{item.address}</Td>
              <Td>
                <Badge colorScheme="teal">{item.blockchain}</Badge>
              </Td>
              <Td>
                <Badge>{item.network}</Badge>
              </Td>
              <Td gap={2} display="flex">
                <IconButton aria-label="copy" icon={<Copy01 />} />
                <IconButton aria-label="delete" icon={<Trash01 />} />
              </Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </Screen>
  );
}