import { BrowserRouter, Route, Routes } from "react-router-dom";
import MnemonicScreen from "./screens/create-wallet.tsx";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route index={true} path="/" element={<MnemonicScreen />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;