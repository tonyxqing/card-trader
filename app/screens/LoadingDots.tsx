import {View, Text, StyleSheet} from 'react-native';
import React from 'react';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withDelay,
  withRepeat,
  withTiming,
} from 'react-native-reanimated';
const CIRCLE_LENGTH = 10;

const LoadingDots = () => {
  const dot1 = useSharedValue(CIRCLE_LENGTH);
  const dot2 = useSharedValue(CIRCLE_LENGTH);
  const dot3 = useSharedValue(CIRCLE_LENGTH);

  const animation1 = useAnimatedStyle(() => {
    return {
      width: dot1.value,
      borderRadius: dot1.value / 2,
    };
  });
  const animation2 = useAnimatedStyle(() => {
    return {
      width: dot2.value,
      borderRadius: dot2.value / 2,
    };
  });
  const animation3 = useAnimatedStyle(() => {
    return {
      width: dot3.value,
      borderRadius: dot3.value / 2,
    };
  });
  React.useEffect(() => {
    dot1.value = withDelay(
      333,
      withRepeat(withTiming(0, {duration: 500}), -1, true),
    );
    dot2.value = withDelay(
      666,
      withRepeat(withTiming(0, {duration: 500}), -1, true),
    );
    dot3.value = withDelay(
      999,
      withRepeat(withTiming(0, {duration: 500}), -1, true),
    );
  }, []);
  return (
    <View className="absolute h-full w-full items-center flex-row justify-center gap-2">
      <Animated.View style={[styles.circle, animation1]}></Animated.View>
      <Animated.View style={[styles.circle, animation2]}></Animated.View>
      <Animated.View style={[styles.circle, animation3]}></Animated.View>
    </View>
  );
};

export default LoadingDots;

const styles = StyleSheet.create({
  circle: {
    aspectRatio: 1,
    height: CIRCLE_LENGTH,
    borderRadius: CIRCLE_LENGTH / 2,
    backgroundColor: 'black',
  },
});
