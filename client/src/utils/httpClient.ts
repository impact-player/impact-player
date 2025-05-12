import axios from 'axios';
import { Depth, KLine, TradesResponse } from './types';

const BASE_URL = 'http://localhost:8080/api/v1';

export async function getDepth(market: string): Promise<Depth> {
  const response = await axios.get(`${BASE_URL}/depth?market=${market}`);
  return response.data;
}

export async function getTrades(market: string): Promise<TradesResponse> {
  const response = await axios.get(`${BASE_URL}/trades?market=${market}`);
  return response.data;
}

export async function getKlines(
  market: string,
  interval: string,
  startTime: string,
  endTime: string
): Promise<KLine[]> {
  console.log('start time', startTime);
  console.log('end time', endTime);
  const response = await axios.get(
    `${BASE_URL}/klines?market=${market}&interval=${interval}&startTime=${startTime}&endTime=${endTime}`
  );

  const data: KLine[] = response.data.data;
  console.log('internal data: ', response.data);
  return data.sort((x, y) => (Number(x.end) < Number(y.end) ? -1 : 1));
}
