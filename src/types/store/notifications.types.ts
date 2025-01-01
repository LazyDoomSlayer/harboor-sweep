export interface IToastNotification {
  id: string;
  title: string;
  description?: string;

  // Icons
  prependIcon?: string;
  appendIcon?: string;
  closeIcon?: string;
  // Styling
  textColor?: string;
  backgroundColor?: string;
  // Custom timeout
  duration?: number;
}

export interface IUseNotificationsStoreState {
  toastNotifications: IToastNotification[];
}

export enum EUseNotificationsStoreActions {
  ADD_TOAST_NOTICATION = 'ADD_TOAST_NOTICATION',
  REMOVE_TOAST_NOTIFICATION = 'REMOVE_TOAST_NOTIFICATION',
}
