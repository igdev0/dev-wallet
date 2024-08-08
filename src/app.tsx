import { Badge, Box, Button, Flex, Heading, Text } from "@chakra-ui/react";
import useMnemonics from "./hooks/use-mnemonics.ts";
import Loading from "./components/loading.tsx";
import Error from "./components/error.tsx";

function App() {
  const mnemonics = useMnemonics();
  if (mnemonics.isLoading) {
    return <Loading />;
  }
  if (mnemonics.error) {
    return <Error error={mnemonics.error} />;
  }

  return (
    <Flex p={3} flexDirection="column" justifyContent="space-between" h="100vh">
      <Box>
        <Heading>Your Recovery code</Heading>
        <Text>
          Please save your recovery code somewhere safe, this will be used to
          generate the seed and the keys necessary to receive and spend coins.
        </Text>
        <Box py={2} gap={1} display="flex" flexWrap="wrap">
          {mnemonics.data!.split(" ").map((v) => (
            <Badge key={v} variant="solid" colorScheme="teal">
              {v}
            </Badge>
          ))}
        </Box>
      </Box>
      <Flex gap={2} justifyContent="flex-end" alignSelf="flex-end">
        <Button colorScheme="teal" onClick={mnemonics.refetch}>
          Regenerate
        </Button>
        <Button colorScheme="blue">Continue</Button>
      </Flex>
    </Flex>
  );
}

export default App;