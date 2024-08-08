import { Box, Spinner } from "@chakra-ui/react";

export default function Loading() {
  return (
    <Box
      bg="teal.100"
      w="100%"
      h="100vh"
      display="flex"
      justifyContent="center"
      alignItems="center"
    >
      <Spinner />
    </Box>
  );
}