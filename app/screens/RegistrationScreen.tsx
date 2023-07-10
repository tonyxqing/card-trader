import {
  StyleSheet,
  Text,
  TextInput,
  View,
  TouchableWithoutFeedback,
  Keyboard,
  Button,
  TouchableOpacity,
  KeyboardAvoidingView,
} from 'react-native';
import React from 'react';
import CheckBox from '@react-native-community/checkbox';
import {RootStackScreenProps} from '../navigation/types';
import {ScrollView} from 'react-native-gesture-handler';
import {SafeAreaView} from 'react-native-safe-area-context';

type Props = {};

const RegistrationScreen = ({
  navigation,
}: RootStackScreenProps<'Registration'>) => {
  const [username, setUsername] = React.useState('');
  const [emailAddr, setEmailAddr] = React.useState('');
  const [passcode, setPasscode] = React.useState('');

  const [agreedToEula, setAgreedToEula] = React.useState(false);
  const emailAddrRef = React.useRef<TextInput>(null);
  const passcodeRef = React.useRef<TextInput>(null);
  return (
    <ScrollView className='flex-1'>
      <SafeAreaView className="h-full">
        <KeyboardAvoidingView enabled behavior="position" className="h-full">
          <View className='flex-1'>
            <View className="px-10 items-start">
              <Text className="text-lg">Username</Text>
              <TextInput
                autoCorrect={false}
                onChangeText={setUsername}
                returnKeyType="next"
                className="w-full p-2 h-14 border-black border-2"
                onSubmitEditing={() => {
                  setTimeout(() => {
                    if (emailAddrRef.current) emailAddrRef.current.focus();
                  }, 0);
                }}
              />
            </View>
            <View className="px-10 items-start">
              <Text className="text-lg">Email</Text>
              <TextInput
                ref={emailAddrRef}
                autoCorrect={false}
                onChangeText={setEmailAddr}
                keyboardType="numeric"
                returnKeyType="done"
                textContentType="emailAddress"
                className="w-full p-2 h-14 border-black border-2"
                onSubmitEditing={() => {
                  setTimeout(() => {
                    if (passcodeRef.current) passcodeRef.current.focus();
                  }, 0);
                }}
              />
            </View>
            <View className="px-10 items-start">
              <Text className="text-lg">5-digit Passcode</Text>
              <TextInput
                ref={passcodeRef}
                autoCorrect={false}
                onChangeText={setPasscode}
                keyboardType="numeric"
                returnKeyType="done"
                maxLength={5}
                textContentType="newPassword"
                className="w-full p-2 h-14 border-black border-2"
              />
            </View>
          </View>
        </KeyboardAvoidingView>
        <View>
          <View className="self-center m-4 gap-2 flex-row">
            <CheckBox onValueChange={setAgreedToEula} />
            <View>
              <Text>I have read and agree to the</Text>
              <TouchableOpacity
                onPress={() => {
                  navigation.navigate('Eula');
                }}>
                <Text className="text-blue-800">Terms of Services.</Text>
              </TouchableOpacity>
            </View>
          </View>
          <View className="rounded border-2 p-2 w-1/2 self-center">
            <Button
              disabled={!agreedToEula || !username || !emailAddr || !passcode}
              title="Create Account"
              onPress={async () => {
                const response = await fetch(
                  'https://1841-204-14-36-68.ngrok-free.app/api/register-player',
                  {
                    method: 'POST',
                    headers: {
                      'ngrok-skip-browser-warning': '1',
                      Accept: 'application/json',
                      'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                      username,
                      email: emailAddr,
                      passcode,
                    }),
                  },
                ).catch(function (error) {
                  console.log(
                    'There has been a problem with your fetch operation: ' +
                      error.message,
                  );
                  // ADD THIS THROW error
                  throw error;
                });
                const data = await response.text();
                console.log(data);
                navigation.navigate('Login');
              }}
            />
          </View>
        </View>
      </SafeAreaView>
    </ScrollView>
  );
};

export default RegistrationScreen;

const styles = StyleSheet.create({});
