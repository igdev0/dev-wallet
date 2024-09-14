import { BrowserRouter, Route, Routes } from "react-router-dom";
import CreateWalletScreen from "./screens/create-wallet.tsx";
import { useAppState } from "./state.ts";
import AuthenticateScreen from "./screens/authenticate.tsx";
import AccountsScreen from "./screens/accounts.tsx";
import { Suspense } from "react";
import Loading from "./components/loading.tsx";
import { QueryErrorResetBoundary } from "react-query";
import { ErrorBoundary } from "react-error-boundary";
import fallbackError from "./components/fallback-error.tsx";

function App() {
  const appState = useAppState();
  return (
    <BrowserRouter>
      <Routes>
        <Route index={true} path="/" element={<CreateWalletScreen />} />
        <Route path="/authenticate" element={<AuthenticateScreen />} />
        <Route
          path="/accounts/:wallet_id"
          element={
            <QueryErrorResetBoundary>
              <ErrorBoundary fallbackRender={fallbackError}>
                <Suspense fallback={<Loading />}>
                  <AccountsScreen />
                </Suspense>
              </ErrorBoundary>
            </QueryErrorResetBoundary>
          }
        />
      </Routes>
    </BrowserRouter>
  );
}

export default App;