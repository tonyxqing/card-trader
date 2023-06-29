import { MaterialTopTabScreenProps, } from "@react-navigation/material-top-tabs";
import { CompositeScreenProps, NavigatorScreenParams } from "@react-navigation/native";
import { NativeStackScreenProps} from "@react-navigation/native-stack";

export type RootTabParamList = {
    Collection: {
      update: number;
    };
    Card: {
      picture: any;
    };
    Camera: undefined;
  };

export type RootStackParamList = {
    Main: NavigatorScreenParams<RootTabParamList>,
    Battle: undefined,
    Train: undefined,
    Shop: undefined,
    Login: undefined,
    Registration: undefined,
    Eula: undefined,
}

export type RootStackScreenProps<T extends keyof RootStackParamList> =
  NativeStackScreenProps<RootStackParamList, T>;

export type RootTabScreenProps<T extends keyof RootTabParamList> = CompositeScreenProps<
    MaterialTopTabScreenProps<RootTabParamList, T>,
    RootStackScreenProps<keyof RootStackParamList>
>;

declare global {
  namespace ReactNavigation {
    interface RootParamList extends RootStackParamList {}
  }
}


