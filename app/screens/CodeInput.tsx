import {Keyboard, SafeAreaView, Text, TextInput, View} from 'react-native';
import React from 'react';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {RootStackParamList} from '../navigation/types';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withSequence,
  withTiming,
} from 'react-native-reanimated';

type Props = {
  maxLength: number;
  username: string;
  handleValidCode: () => void;
};

async function isValidCode(username: string, passcode: string) {
  const response = await fetch(
    'https://1841-204-14-36-68.ngrok-free.app/api/authenticate-player',
    {
      method: 'POST',
      headers: {
        'ngrok-skip-browser-warning': '1',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        passcode,
      }),
    },
  ).catch(function (error) {
    console.log(
      'There has been a problem with your fetch operation: ' + error.message,
    );
    // ADD THIS THROW error
    throw error;
  });
  const data = await response.text();

 
  if (data === 'Login Passed') {
    return true;
  }
  return false;
}

const CodeInput = ({maxLength, handleValidCode, username}: Props) => {
  const [codeString, setCodeString] = React.useState('');
  const [failedAttempts, setFailedAttempts] = React.useState(0);
  const shakePosition = useSharedValue(0);

  const animatedStyles = useAnimatedStyle(() => {
    return {
      transform: [
        {
          translateX: shakePosition.value,
        },
      ],
    };
  });

  function failedAttemptAnimation() {
    shakePosition.value = withRepeat(
      withSequence(
        withTiming(4, {duration: 30}),
        withTiming(-4, {duration: 30}),
        withTiming(0, {duration: 30}),
      ),
      3,
      false,
    );
  }

  return (
    <SafeAreaView className="flex-1 h-full items-center justify-center">
      <Animated.View className="items-center" style={animatedStyles}>
        <View className="flex-row">
          {Array.from(Array(maxLength).keys()).map((_, i) => {
            if (codeString[i]) {
              return (
                <View className="flex-1 m-2 h-14 items-center justify-center border-solid border-opacity-100 border-black border-2">
                  <Text className="text-xl">•</Text>
                </View>
              );
            } else {
              return (
                <View className="flex-1 m-2 h-14 border-solid border-opacity-100 border-black border-2" />
              );
            }
          })}
        </View>
      </Animated.View>

      <TextInput
        className="opacity-0 w-full h-full self-center absolute"
        onChangeText={async (code) => {
          if (code.length <= maxLength) {
            setCodeString(code);
            // validate the code
            if (code.length === maxLength) {
              setTimeout(() => {}, 3000);
              if (await isValidCode(username, code)) {
                setFailedAttempts(0);
                handleValidCode();
              } else {
                setFailedAttempts(attempts => attempts + 1);
                failedAttemptAnimation();
              }
            }
          }
        }}
        maxLength={6}
        value={codeString}
        keyboardType="numeric"
        placeholder="key"
        returnKeyType="send"
      />
      <Text
        className={`absolute -bottom-8  text-red-600 ${
          failedAttempts === 0 && 'opacity-0'
        }`}>
        Could not find username or passcode
      </Text>
    </SafeAreaView>
  );
};

export default CodeInput;
