export interface IPort {
  id: string;
  port: string;
}

export interface IProcess {
  pid: number;
  process_name: string;
  process_path: string;
}

export type TPortProcessItem = IPort & IProcess;
export type TPortProcessList = TPortProcessItem[];
