import { create } from "zustand";

export interface AppState {
  wallet_authenticated: string | null;
  setAuthenticated: (name: string) => void;
}

export const useAppState = create<AppState>((setState, getState, store) => ({
  wallet_authenticated: null,
  setAuthenticated(wallet_name: string) {
    setState({ wallet_authenticated: wallet_name });
  },
  logout() {
    setState({ wallet_authenticated: null });
  },
}));