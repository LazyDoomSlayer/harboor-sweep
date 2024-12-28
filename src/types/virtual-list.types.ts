export enum EScrollBehavior {
  AUTO = 'auto',
  SMOOTH = 'smooth',
}

export interface IProps {
  list: unknown[];
  scrollBehavior?: EScrollBehavior;
  itemHeight: number;
}
