import React from "react";
import ReactDOM from "react-dom/client";

import App from "./app.tsx";
import { ChakraProvider } from "@chakra-ui/react";
import { theme } from "./theme";
import { QueryClient, QueryClientProvider } from "react-query";

const client = new QueryClient();
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={client}>
      <ChakraProvider theme={theme}>
        <App />
      </ChakraProvider>
    </QueryClientProvider>
  </React.StrictMode>,
);