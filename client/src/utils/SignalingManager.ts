import { DepthPayload, TradePayload } from './types';

export const BASE_URL = 'ws://localhost:8081/ws';

type Callback<T> = (data: T) => void;

type RoomType = 'depth' | 'trade';
type Room = `${RoomType}@${string}`;

export class SignalingManager {
  private ws: WebSocket;
  private static instance: SignalingManager;
  private buffer: any[] = [];
  private depthCallbacks: Record<string, Callback<DepthPayload>[]> = {};
  private tradeCallbacks: Record<string, Callback<TradePayload>[]> = {};
  private id = 1;
  private opened = false;

  private constructor() {
    this.ws = new WebSocket(BASE_URL);
    this.init();
  }

  public static getInstance(): SignalingManager {
    if (!this.instance) {
      this.instance = new SignalingManager();
    }
    return this.instance;
  }

  private init() {
    this.ws.onopen = () => {
      this.opened = true;
      this.buffer.forEach((m) => this.ws.send(JSON.stringify(m)));
      this.buffer = [];
    };

    this.ws.onmessage = (evt) => {
      const env = JSON.parse(evt.data);
      const room: string = env.room;

      if (!room) return;

      try {
        const data = JSON.parse(env.data);

        if (room.startsWith('depth@') && this.depthCallbacks[room]?.length) {
          const { a: asks, b: bids } = data.data || {};

          if (asks !== undefined && bids !== undefined) {
            this.depthCallbacks[room].forEach((cb) => cb({ bids, asks }));
          }
        } else if (
          room.startsWith('trade@') &&
          this.tradeCallbacks[room]?.length
        ) {
          const { price, quantity, side, timestamp } = data;

          if (price !== undefined) {
            this.tradeCallbacks[room].forEach((cb) =>
              cb({ price, quantity, side, timestamp })
            );
          }
        }
      } catch (error) {
        console.error('Error processing message:', error);
      }
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  public subscribe(room: Room) {
    const message = {
      type: 'SUBSCRIBE',
      payload: { room },
    };
    this.sendMessage(message);
  }

  public unsubscribe(room: Room) {
    const message = {
      type: 'UNSUBSCRIBE',
      payload: { room },
    };
    this.sendMessage(message);
  }

  public sendMessage(msg: any) {
    const withId = { ...msg, id: this.id++ };
    if (!this.opened) {
      this.buffer.push(withId);
    } else {
      this.ws.send(JSON.stringify(withId));
    }
  }

  public registerDepthCallback(room: string, cb: Callback<DepthPayload>) {
    (this.depthCallbacks[room] ||= []).push(cb);
  }

  public registerTradeCallback(room: string, cb: Callback<TradePayload>) {
    (this.tradeCallbacks[room] ||= []).push(cb);
  }

  public deRegisterDepthCallback(room: string, cb: Callback<DepthPayload>) {
    if (!this.depthCallbacks[room]) return;
    this.depthCallbacks[room] = this.depthCallbacks[room].filter(
      (x) => x !== cb
    );
  }

  public deRegisterTradeCallback(room: string, cb: Callback<TradePayload>) {
    if (!this.tradeCallbacks[room]) return;
    this.tradeCallbacks[room] = this.tradeCallbacks[room].filter(
      (x) => x !== cb
    );
  }

  public registerCallback<T extends RoomType>(
    room: `${T}@${string}`,
    cb: T extends 'depth' ? Callback<DepthPayload> : Callback<TradePayload>
  ) {
    if (room.startsWith('depth@')) {
      this.registerDepthCallback(room, cb as Callback<DepthPayload>);
    } else if (room.startsWith('trade@')) {
      this.registerTradeCallback(room, cb as Callback<TradePayload>);
    }
  }

  public deRegisterCallback<T extends RoomType>(
    room: `${T}@${string}`,
    cb: T extends 'depth' ? Callback<DepthPayload> : Callback<TradePayload>
  ) {
    if (room.startsWith('depth@')) {
      this.deRegisterDepthCallback(room, cb as Callback<DepthPayload>);
    } else if (room.startsWith('trade@')) {
      this.deRegisterTradeCallback(room, cb as Callback<TradePayload>);
    }
  }
}
