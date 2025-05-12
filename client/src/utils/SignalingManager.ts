// src/utils/SignalingManager.ts
export const BASE_URL = 'ws://localhost:8081/ws';

type DepthPayload = { bids: [string, string][]; asks: [string, string][] };
type Callback = (data: DepthPayload) => void;

export class SignalingManager {
  private ws: WebSocket;
  private static instance: SignalingManager;
  private buffer: any[] = [];
  private callbacks: Record<string, Callback[]> = {};
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
      if (!this.callbacks[room]?.length) return;

      // parse the stringified data field
      const wrapper = JSON.parse(env.data);
      const { a: asks, b: bids } = wrapper.data;

      this.callbacks[room].forEach((cb) => cb({ bids, asks }));
    };
  }

  public sendMessage(msg: any) {
    const withId = { ...msg, id: this.id++ };
    if (!this.opened) {
      this.buffer.push(withId);
    } else {
      this.ws.send(JSON.stringify(withId));
    }
  }

  public registerCallback(room: string, cb: Callback) {
    (this.callbacks[room] ||= []).push(cb);
  }

  public deRegisterCallback(room: string, cb: Callback) {
    if (!this.callbacks[room]) return;
    this.callbacks[room] = this.callbacks[room].filter((x) => x !== cb);
  }
}
