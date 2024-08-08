import { Badge, Box, Button, Flex, Heading, Text } from "@chakra-ui/react";
import useMnemonics from "../hooks/use-mnemonics.ts";
import Loading from "../components/loading.tsx";
import Error from "../components/error.tsx";
import Screen from "../components/screen.tsx";

export default function MnemonicScreen() {
  const mnemonics = useMnemonics();
  if (mnemonics.isLoading) {
    return <Loading />;
  }
  if (mnemonics.error) {
    return <Error error={mnemonics.error} />;
  }
  return (
    <Screen title="Setting up / recovery code">
      <Heading textAlign="center" mb={1} mt={4}>
        Your Recovery code
      </Heading>
      <Text mb={4} textAlign="center">
        Please save your recovery code somewhere safe, this will be used to
        generate the seed and the keys necessary to receive and spend coins.
      </Text>
      <Box
        py={2}
        gap={1}
        display="flex"
        flexWrap="wrap"
        justifyContent="center"
      >
        {mnemonics.data!.split(" ").map((v, i) => (
          <Badge key={`${v}-${i}`} colorScheme="teal">
            {v}
          </Badge>
        ))}
      </Box>
      <Flex
        gap={2}
        justifyContent="center"
        alignSelf="flex-end"
        position="absolute"
        bottom={3}
        w="100%"
      >
        <Button colorScheme="teal" onClick={mnemonics.refetch}>
          Regenerate
        </Button>
        <Button colorScheme="blue">Continue</Button>
      </Flex>
    </Screen>
  );
}