// import React from 'react';
// import {View, Text, StyleSheet, Dimensions} from 'react-native';
// import {LinearGradient} from 'expo-linear-gradient';
// import {
//   PanGestureHandler,
//   PanGestureHandlerGestureEvent,
// } from 'react-native-gesture-handler';
// import Animated, {
//   useSharedValue,
//   useAnimatedStyle,
//   withSpring,
//   useAnimatedGestureHandler,
//   runOnJS,
// } from 'react-native-reanimated';

// // const windowHeight = Dimensions.get('window').height;
// // const windowWidth = Dimensions.get('window').width;

// const BattleScreen = () => {
//   const touchX = useSharedValue(0);
//   const touchY = useSharedValue(0);
//   const r = useSharedValue(80);
//   const br = useSharedValue(40);

//   const gestureHandler = useAnimatedGestureHandler<
//     PanGestureHandlerGestureEvent,
//     {touchX: number; touchY: number; r: number; br: number}
//   >({
//     onStart: (_, ctx) => {
//       ctx.touchX = touchX.value;
//       ctx.touchY = touchY.value;
//       ctx.br = br.value;
//       ctx.r = r.value;
//     },
//     onActive: ({translationX, translationY}, ctx) => {
//       touchX.value = ctx.touchX + translationX;
//       touchY.value = ctx.touchY + translationY;
//     },

//     onEnd: ({velocityX, velocityY}, ctx) => {
//       // need calculate distance between original and current points get magnitude

//       let distX = ctx.touchX - touchX.value;
//       let distY = ctx.touchY - touchY.value;

//       let m = distX ** 2 + distY ** 2;
//       let magnitude = Math.sqrt(m);

//       // multiply magnitude by the direction vectors

//       touchY.value = withSpring(touchY.value + distY * 0.01 * magnitude);
//       touchX.value = withSpring(touchX.value + distX * 0.01 * magnitude);

//       ctx.touchX = touchX.value;
//       ctx.touchY = touchY.value;

//       // touchX.value = withSpring(ctx.touchX);
//       // touchY.value = withSpring(ctx.touchY);
//       // r.value = withSpring(80);
//       // br.value = withSpring(40);
//     },
//   });

//   const animatedStyle = useAnimatedStyle(() => {
//     console.log(touchX.value, touchY.value);
//     return {
//       height: r.value,
//       borderRadius: br.value,
//       transform: [{translateX: touchX.value}, {translateY: touchY.value}],
//     };
//   });

//   return (
//     <View style={styles.container}>
//       <LinearGradient
//         colors={['#fca817', '#fcda17']}
//         style={styles.background}
//       />
//       <View style={styles.animatedContainer}>
//         <PanGestureHandler onGestureEvent={gestureHandler}>
//           <Animated.View style={[animatedStyle, styles.circle]} />
//         </PanGestureHandler>
//       </View>
//     </View>
//   );
// };

// export default BattleScreen;

// const styles = StyleSheet.create({
//   container: {
//     display: 'flex',
//     flex: 1,
//   },
//   background: {
//     position: 'absolute',
//     left: 0,
//     top: 0,
//     right: 0,
//     bottom: 0,
//   },
//   circle: {
//     aspectRatio: 1,
//     borderRadius: 40,
//     borderWidth: 2,
//     borderColor: 'black',
//     alignSelf: 'center',
//   },
//   animatedContainer: {
//     display: 'flex',
//     justifyContent: 'center',
//     height: '100%',
//     width: '100%',
//   },
// });

import {StyleSheet, Text, View} from 'react-native';
import React from 'react';

const BattleScreen = () => {
  return (
    <View>
      <Text>BattleScreen</Text>
    </View>
  );
};

export default BattleScreen;

const styles = StyleSheet.create({});
