import axios from 'axios';
import { DepthResponse } from './types';

const BASE_URL = 'http://localhost:8080/api/v1';

export async function getDepth(market: string): Promise<DepthResponse> {
  const response = await axios.get(`${BASE_URL}/depth?market=${market}`);
  return response.data;
}
