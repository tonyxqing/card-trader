import {
  Keyboard,
  StyleSheet,
  Text,
  TextInput,
  TouchableWithoutFeedback,
  View,
} from 'react-native';
import React from 'react';
import CodeInput from './CodeInput';
import {
  RootStackParamList,
  RootStackScreenProps,
  RootTabScreenProps,
} from '../navigation/types';
import {TouchableOpacity} from 'react-native';
import {RouteProp, useRoute} from '@react-navigation/native';
import LoadingDots from './LoadingDots';
type MainScreenRouteProps = RouteProp<RootStackParamList, 'Main'>;

const LoginScreen = ({navigation}: RootStackScreenProps<'Login'>) => {
  const [username, setUsername] = React.useState('');
  const route = useRoute<MainScreenRouteProps>();

  return (
    <TouchableWithoutFeedback
      onPress={() => {
        Keyboard.dismiss();
      }}>
      <View className="flex-1">
        <View className="flex justify-center">
          <View className="px-10 h-1/4  items-start">
            <Text className="text-lg">Username</Text>
            <TextInput
              autoCorrect={false}
              onChangeText={setUsername}
              className="w-full p-2 h-14 border-black border-2"></TextInput>
          </View>
          <View className="h-1/4  px-10 w-full items-start">
            <Text className="py-2 text-lg">Passcode</Text>
            <CodeInput
              username={username}
              maxLength={5}
              handleValidCode={() => {
                Keyboard.dismiss();
                navigation.navigate('Main', {...route.params});
              }}
            />
          </View>
        </View>
        <LoadingDots/>
        <View className="flex-row shadow-inner justify-center ">
          <Text>Don't have an account? </Text>
          <TouchableOpacity
            onPress={() => {
              navigation.navigate('Registration');
            }}>
            <Text className="text-blue-800">Register here.</Text>
          </TouchableOpacity>
        </View>
      </View>
    </TouchableWithoutFeedback>
  );
};

export default LoginScreen;

const styles = StyleSheet.create({});
