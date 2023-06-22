import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  Dimensions,
  TouchableOpacity,
} from 'react-native';
import {LinearGradient} from 'expo-linear-gradient';
import Animated, {
  withSpring,
  withTiming,
  useSharedValue,
  useAnimatedStyle,
} from 'react-native-reanimated';

const colorArray = [
  '#FF6633',
  '#FFB399',
  '#FF33FF',
  '#FFFF99',
  '#00B3E6',
  '#E6B333',
  '#3366E6',
  '#999966',
  '#99FF99',
  '#B34D4D',
  '#80B300',
  '#809900',
  '#E6B3B3',
  '#6680B3',
  '#66991A',
  '#FF99E6',
  '#CCFF1A',
  '#FF1A66',
  '#E6331A',
  '#33FFCC',
  '#66994D',
  '#B366CC',
  '#4D8000',
  '#B33300',
  '#CC80CC',
  '#66664D',
  '#991AFF',
  '#E666FF',
  '#4DB3FF',
  '#1AB399',
  '#E666B3',
  '#33991A',
  '#CC9999',
  '#B3B31A',
  '#00E680',
  '#4D8066',
  '#809980',
  '#E6FF80',
  '#1AFF33',
  '#999933',
  '#FF3380',
  '#CCCC00',
  '#66E64D',
  '#4D80CC',
  '#9900B3',
  '#E64D66',
  '#4DB380',
  '#FF4D4D',
  '#99E6E6',
  '#6666FF',
];

const windowWidth = Dimensions.get('window').width;
const windowHeight = Dimensions.get('window').height;

const getRandomX = (width: number) =>
  Math.random() * (Math.random() > 0.5 ? 1 : -1) * (width / 2 - 40);
const getRandomY = (height: number) =>
  Math.random() * (Math.random() > 0.5 ? 1 : -1) * (height / 2 - 40);

const TrainingScreen = () => {
  const [comboCounter, setComboCounter] = React.useState(0);
  const [score, setScore] = React.useState(0);
  const color = useSharedValue(
    colorArray[Math.floor(Math.random() * colorArray.length)],
  );
  const r = useSharedValue(80);
  const br = useSharedValue(40);
  const x = useSharedValue(getRandomX(windowWidth));
  const y = useSharedValue(getRandomY(windowHeight));
  const animatedStyle = useAnimatedStyle(() => {
    return {
      backgroundColor: color.value,
      height: r.value,
      borderRadius: br.value,
      transform: [
        {
          translateX: x.value,
        },
        {
          translateY: y.value,
        },
      ],
    };
  });

  React.useEffect(() => {
    let intervals: number[] = [];
    const wrapper = () => {
      x.value = withTiming(getRandomX(windowWidth), {duration: 300});
      y.value = withTiming(getRandomY(windowHeight), {duration: 300});
      const id = setTimeout(wrapper, 1000);
      intervals.push(id);
      return id;
    };
    const comboInterval = setTimeout(() => {
      setComboCounter(0);
      wrapper();
    }, 1000);

    return () => {
      clearInterval(comboInterval);
      intervals.forEach(e => clearInterval(e));
    };
  }, [comboCounter]);

  return (
    <View style={styles.container}>
      <LinearGradient
        colors={['#8cc0ff', '#172efc']}
        style={styles.background}
      />
      <Text
        style={{
          position: 'absolute',
          fontWeight: 'bold',
          fontSize: 24,
          height: '100%',
          width: '100%',
          padding: 10,
        }}>
        {comboCounter ? `${comboCounter}x` : ''}
      </Text>
      <Text
        style={{
          position: 'absolute',
          fontWeight: 'bold',
          fontSize: 24,
          height: '100%',
          width: '100%',
          textAlign: 'right',
          padding: 10,
        }}>
        Score: {score}
      </Text>
      <Animated.View style={[styles.circle, animatedStyle]}>
        <TouchableOpacity
          style={{height: '100%', width: '100%'}}
          onPress={() => {
            setComboCounter(prev => prev + 1);
            r.value = withSpring(20);
            br.value = withSpring(10);
            x.value = withTiming(getRandomX(windowWidth), {duration: 300});
            y.value = withTiming(
              getRandomY(windowHeight),
              {duration: 300},
              () => {
                color.value =
                  colorArray[Math.floor(Math.random() * colorArray.length)];
                r.value = withSpring(80);
                br.value = withSpring(40);
              },
            );
            setScore(prev => prev + 1 + 1 * comboCounter);
          }}></TouchableOpacity>
      </Animated.View>
    </View>
  );
};

export default TrainingScreen;

const styles = StyleSheet.create({
  container: {
    display: 'flex',
    justifyContent: 'center',
    height: '100%',
    width: '100%',
  },
  background: {
    position: 'absolute',
    left: 0,
    top: 0,
    right: 0,
    bottom: 0,
    zIndex: -10,
  },
  circle: {
    aspectRatio: 1,
    borderColor: 'black',
    borderWidth: 2,
    alignSelf: 'center',
  },
});
