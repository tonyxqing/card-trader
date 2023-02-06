import * as React from 'react';
import { StyleSheet } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createMaterialTopTabNavigator } from '@react-navigation/material-top-tabs';
import { createNativeStackNavigator} from '@react-navigation/native-stack'
import CollectionScreen from './screens/CollectionScreen';
import CardScreen from './screens/CardScreen';
import CameraScreen from './screens/CameraScreen';
import { RootTabParamList } from './routing';
import * as ScreenOrientation from 'expo-screen-orientation';
import BattleScreen from './screens/BattleScreen'
import TrainingScreen from './screens/TrainingScreen'
import ShopScreen from './screens/ShopScreen'

  const Tab = createMaterialTopTabNavigator<RootTabParamList>();
  const Stack = createNativeStackNavigator();
export default function App() {

  React.useEffect(() => {
    async function changeScreenOrientation() {
      await ScreenOrientation.lockAsync(3);
    }
    changeScreenOrientation();
  }, []);

  const TabNavigator =  () => (<Tab.Navigator initialRouteName="Card" tabBarPosition="bottom">
        <Tab.Screen name="Collection" component={CollectionScreen}></Tab.Screen>
        <Tab.Screen name="Card" component={CardScreen}></Tab.Screen>
        <Tab.Screen name="Camera" component={CameraScreen}></Tab.Screen>
      </Tab.Navigator>)
    
    


  return (
    <NavigationContainer>
    <Stack.Navigator>
      <Stack.Screen name="Main" component={TabNavigator} options={{headerShown: false}}></Stack.Screen>
      <Stack.Screen name="Battle" component={BattleScreen} ></Stack.Screen>
      <Stack.Screen name="Train" component={TrainingScreen} ></Stack.Screen>
      <Stack.Screen name="Shop" component={ShopScreen} ></Stack.Screen>
    </Stack.Navigator>
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({});
