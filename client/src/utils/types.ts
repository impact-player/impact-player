interface OrderDetails {
  quantity: string;
  type_: 'Bid' | 'Ask';
}

export interface DepthResponse {
  type: string;
  payload: {
    orders: {
      [price: string]: OrderDetails;
    };
  };
}
