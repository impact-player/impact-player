export interface KLine {
  close: string;
  end: string;
  high: string;
  low: string;
  open: string;
  quoteVolume: string;
  start: string;
  trades: string;
  volume: string;
}

export interface Depth {
  payload: {
    bids: [string, string][];
    asks: [string, string][];
  };
}

export interface Trade {
  currency_code: string;
  price: number;
  quantity: number;
  time: string; // ISO 8601 timestamp
  volume: number;
}

export interface TradesResponse {
  success: boolean;
  data: Trade[];
}
