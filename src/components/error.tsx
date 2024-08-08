import { Box, Heading, Text } from "@chakra-ui/react";

export default function Error(props: { error: string }) {
  return (
    <Box
      bg="teal.100"
      w="100%"
      h="100vh"
      display="flex"
      flexDirection="column"
      justifyContent="center"
      alignItems="center"
    >
      <Heading>App error</Heading>
      <Text>{props.error}</Text>
    </Box>
  );
}