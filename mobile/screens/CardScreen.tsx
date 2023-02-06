import React from 'react';
import { View, Text, StyleSheet, Image, TouchableOpacity } from 'react-native';
import { PanGestureHandler } from 'react-native-gesture-handler';
import { RootTabParamList } from '../routing';
import { MaterialTopTabScreenProps } from '@react-navigation/material-top-tabs';
import Animated, {
  useAnimatedStyle,
  useAnimatedProps,
  useSharedValue,
  withSpring,
  useAnimatedGestureHandler,
  interpolate,
  withTiming,
  withRepeat,
  Extrapolation,
} from 'react-native-reanimated';
import { LinearGradient } from 'expo-linear-gradient';
import * as FileSystem from 'expo-file-system';
import CardDetails from '../CardDetails';
import { CardStatistics, CardType, cardTypes } from '../Types';
import AsyncStorage from '@react-native-async-storage/async-storage';
import {
  uniqueNamesGenerator,
  Config,
  adjectives,
  names,
  animals,
  languages,
} from 'unique-names-generator';
type CardScreenProps = MaterialTopTabScreenProps<RootTabParamList, 'Card'>;
export const imageDir = FileSystem.documentDirectory + 'card-trading/images/';
export const statDir = FileSystem.documentDirectory + 'card-trading/stats/';

export const newString = () => {
  const config: Config = {
    dictionaries: [adjectives, animals, names],
    separator: ' ',
    length: 3,
    style: 'capital',
  };

  return uniqueNamesGenerator(config);
};
const CardScreen = ({ navigation, route }: CardScreenProps) => {
  const [cardUri, setCardUri] = React.useState<string>('');
  const [cardStats, setCardStats] = React.useState<CardStatistics>({
    name: newString(),
    cardType: cardTypes[Math.floor(Math.random() * cardTypes.length)],
    hitpoints: Math.floor(Math.random() * 100),
    attack: Math.floor(Math.random() * 100),
    strength: Math.floor(Math.random() * 100),
    defense: Math.floor(Math.random() * 100),
  });

  const ensureStatsExistsOrAdd = async (filename: string) => {
    const response = await AsyncStorage.getItem(filename);
    if (!response || !response.name) {
      const stats: CardStatistics = {
        name: newString(),
        cardType: cardTypes[Math.floor(Math.random() * cardTypes.length)],
        hitpoints: Math.floor(Math.random() * 100),
        attack: Math.floor(Math.random() * 100),
        strength: Math.floor(Math.random() * 100),
        defense: Math.floor(Math.random() * 100),
      };
      const statString = JSON.stringify(stats);

      await AsyncStorage.setItem(filename, statString);
      setCardStats(stats);
      return;
    }
    const json = JSON.parse(response);
    setCardStats(json);
  };

  const rollStats = async () => {
    const stats: CardStatistics = {
      name: newString(),
      cardType: cardTypes[Math.floor(Math.random() * cardTypes.length)],
      hitpoints: Math.floor(Math.random() * 100),
      attack: Math.floor(Math.random() * 100),
      strength: Math.floor(Math.random() * 100),
      defense: Math.floor(Math.random() * 100),
    };
    await AsyncStorage.setItem(cardUri, JSON.stringify(stats));
    setCardStats(stats);
  };

  React.useEffect(() => {
    const baseImage = require('../assets/27.jpg');
    const ensureDirExists = async (dir: string) => {
      const dirInfo = await FileSystem.getInfoAsync(dir);
      if (!dirInfo.exists) {
        await FileSystem.makeDirectoryAsync(dir, { intermediates: true });
      }
    };
    const ensureImageExistsOrAdd = async (
      dir: string,
      filename: string,
      uri: string
    ) => {
      console.log('ensuring image exists');

      const dirInfo = await FileSystem.getInfoAsync(dir + filename);
      if (!dirInfo.exists) {
        console.log('imag not exists');

        await FileSystem.downloadAsync(uri, dir + filename);
      }
      setCardUri(filename);
    };
    async function loadModel() {
      await ensureDirExists(imageDir);
      await ensureImageExistsOrAdd(
        imageDir,
        'pengu.jpg',
        Image.resolveAssetSource(baseImage).uri
      );
      await ensureStatsExistsOrAdd('pengu.txt');
    }
    loadModel();
  }, []);

  React.useEffect(() => {
    if (route?.params) {
      setCardUri(route.params.picture);
      ensureStatsExistsOrAdd(route.params.picture);
    }
  }, [route.params]);

  const touchX = useSharedValue(0);
  const touchY = useSharedValue(0);
  const gestureHandler = useAnimatedGestureHandler({
    onActive: (event) => {
      touchX.value = event.translationX;
      touchY.value = event.translationY;
    },
    onEnd: () => {
      touchX.value = withSpring(0);
      touchY.value = withSpring(0);
    },
  });

  const animatedStyles = useAnimatedStyle(() => {
    const rotX = interpolate(-touchY.value, [-100, 100], [-0.9, 0.9], {
      extrapolateLeft: Extrapolation.CLAMP,
      extrapolateRight: Extrapolation.CLAMP,
    });
    const rotY = interpolate(touchX.value, [-100, 100], [-0.9, 0.9], {
      extrapolateLeft: Extrapolation.CLAMP,
      extrapolateRight: Extrapolation.CLAMP,
    });
    return {
      transform: [
        {
          perspective: 1000,
        },
        {
          rotateX: rotX + 'turn',
        },
        {
          rotateY: rotY + 'turn',
        },
      ],
    };
  });

  return (
    <View style={{ display: 'flex', flexDirection: 'column', flex: 1 }}>
      <LinearGradient
        colors={['#ffafbd', '#ffc3a0']}
        start={{ x: 0.2, y: 0 }}
        end={{ x: 0.8, y: 0.0 }}
        style={styles.background}
      />
      <View
        style={[
          styles.container,
          { flex: 3, alignItems: 'center', justifyContent: 'center' },
        ]}>
        <View style={[styles.top, { flex: 2 }]}>
          <View
            style={{
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'space-between',
            }}>
            <Text
              style={{ fontSize: 20, fontWeight: 'bold', paddingBottom: 5 }}>
              {cardStats.name}
            </Text>
            <PanGestureHandler onGestureEvent={gestureHandler}>
              <Animated.View
                style={[
                  {
                    shadowColor: '#888888',
                    shadowOffset: { width: 5, height: 10 },
                    shadowOpacity: 1,
                    shadowRadius: 3,
                    zIndex: 99,
                  },
                  animatedStyles,
                ]}>
                {
                  <Image
                    style={styles.card}
                    resizeMode="cover"
                    source={{ uri: imageDir + cardUri }}
                  />
                }
              </Animated.View>
            </PanGestureHandler>
            <View
              style={{
                display: 'flex',
                flexDirection: 'row',
                justifyContent: 'space-between',
              }}>
              <Text style={{ paddingTop: 10 }}>
                Combat Lvl:{' '}
                {Math.floor(
                  0.25 * (cardStats.defense + cardStats.hitpoints) +
                    0.325 * (cardStats.strength + cardStats.attack)
                )}{' '}
              </Text>
              <Text style={{ paddingTop: 10 }}>({cardStats.cardType})</Text>
            </View>
          </View>
        </View>
        <View style={[styles.top, { flex: 1 }]}>
          <CardDetails {...cardStats} />
        </View>
      </View>

      <View
        style={[
          styles.container,
          {
            flex: 1,
            alignItems: 'center',
            justifyContent: 'space-evenly',
          },
        ]}>
        <View style={styles.buttonWrapper}>
          <LinearGradient colors={['#fca817', '#fcda17']} style={styles.button}>
            <TouchableOpacity onPress={() => {navigation.navigate('Battle')}}>
              <Text
                style={{
                  textAlign: 'center',
                  color: '#ffebbd',
                  fontSize: 24,
                  fontWeight: 'bold',
                }}>
                Battle
              </Text>
            </TouchableOpacity>
          </LinearGradient>
        </View>
        <View style={styles.buttonWrapper}>
          <LinearGradient colors={['#8cc0ff', '#172efc']} style={styles.button}>
            <TouchableOpacity
              onPress={() => {
                navigation.navigate('Train');
              }}>
              <Text
                style={{
                  textAlign: 'center',
                  color: '#d4d8ff',
                  fontSize: 24,
                  fontWeight: 'bold',
                }}>
                Train
              </Text>
            </TouchableOpacity>
          </LinearGradient>
        </View>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'row',
  },
  card: {
    borderRadius: 25 / 1.4,
    width: '100%',
    aspectRatio: 1 / 1.4,
    borderWidth: 3,
  },
  top: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    margin: 8,
    height: '50%',
  },
  background: {
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    height: '100%',
    backfaceVisibility: 'hidden',
    zIndex: -150,
  },
  button: {
    padding: 15,
    borderRadius: 4,
    width: '80%',
    display: 'flex',
  },
  buttonWrapper: {
    display: 'flex',
    flex: 1,
    alignItems: 'center',
    border: '2px solid gray',
  },
});
export default CardScreen;
