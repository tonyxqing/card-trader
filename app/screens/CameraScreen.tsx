import {Camera, CameraType} from 'expo-camera';
import {
  Text,
  View,
  StyleSheet,
  Platform,
  Button,
  TouchableOpacity,
  Dimensions,
} from 'react-native';
import React from 'react';
import {MaterialTopTabScreenProps} from '@react-navigation/material-top-tabs';
import {RootTabScreenProps} from '../navigation/types';
import * as FileSystem from 'expo-file-system';
import {imageDir} from './CardScreen';
import {CardStatistics, cardTypes} from '../Types';
import AsyncStorage from '@react-native-async-storage/async-storage';
import {newString} from './CardScreen';
import Reanimated, {
  Extrapolate,
  interpolate,
  runOnJS,
  useAnimatedProps,
  useSharedValue,
} from 'react-native-reanimated';
import {Gesture, GestureDetector} from 'react-native-gesture-handler';

const AnimatedCamera = Reanimated.createAnimatedComponent(Camera);
Reanimated.addWhitelistedNativeProps({
  zoom: true,
});
const minZoom = 0;
const maxZoom = 1;
const SCALE_FULL_ZOOM = 1;
const CameraScreen = ({navigation, route}: RootTabScreenProps<'Camera'>) => {
  const [type, setType] = React.useState<CameraType>(CameraType.back);
  const [permission, requestPermission] = Camera.useCameraPermissions();
  const [isPreview, setIsPreview] = React.useState(false);
  const [isCameraReady, setIsCameraReady] = React.useState(false);
  const frontCamera = useSharedValue(false);
  const zoom = useSharedValue(0);
  const startZoom = useSharedValue(0);
  const cameraRef = React.useRef(null);
  const [zoomValue, setZoomValue] = React.useState(0);
  const [capturedImage, setCapturedImage] = React.useState('');
  const onCameraReady = () => {
    setIsCameraReady(true);
  };
  const cancelPreview = async () => {
    await cameraRef.current.resumePreview();
    setIsPreview(false);
  };
  const renderCancelPreviewButton = () => (
    <TouchableOpacity onPress={cancelPreview} style={styles.closeButton}>
      <View style={[styles.closeCross, {transform: [{rotate: '45deg'}]}]} />
      <View style={[styles.closeCross, {transform: [{rotate: '-45deg'}]}]} />
    </TouchableOpacity>
  );

  const takePicture = async () => {
    if (cameraRef.current) {
      const options = {
        quality: 0.5,
        base64: true,
        skipProcessing: true,
        isImageMirror: true,
      };
      const data = await cameraRef.current.takePictureAsync(options);
      const source = data.uri;
      if (source) {
        setCapturedImage(source);
        await cameraRef.current.pausePreview();
        setIsPreview(true);
      }
    }
  };

  const keepPicture = async () => {
    const date = new Date();
    const fileName = date.toJSON().replace(/[-:TZ.]/gm, '');
    await FileSystem.moveAsync({from: capturedImage, to: imageDir + fileName});
    const stats: CardStatistics = {
      name: newString(),
      cardType: cardTypes[Math.floor(Math.random() * cardTypes.length)],
      hitpoints: Math.floor(Math.random() * 100),
      attack: Math.floor(Math.random() * 100),
      strength: Math.floor(Math.random() * 100),
      defense: Math.floor(Math.random() * 100),
    };
    const statString = JSON.stringify(stats);

    await AsyncStorage.setItem(fileName, statString);
    await cameraRef.current.resumePreview();
    setIsPreview(false);
    navigation.jumpTo('Card', {picture: fileName});
  };
  if (!permission) {
    // Camera permissions are still loading
    return <View />;
  }

  if (!permission.granted) {
    // Camera permissions are not granted yet
    return (
      <View style={styles.container}>
        <Text style={{textAlign: 'center'}}>
          We need your permission to show the camera
        </Text>
        <Button onPress={requestPermission} title="grant permission" />
      </View>
    );
  }
  function setZoom(scale: number) {
    setZoomValue(scale);
  }
  const pinchZoomGesture = Gesture.Pinch()
    .onBegin(() => {
      startZoom.value = zoom.value;
    })
    .onUpdate(e => {
      const currZoom = startZoom.value ?? 0;
      const scale = interpolate(
        e.scale,
        [0.75, 1, 4],
        [-1, 0, 1],
        Extrapolate.CLAMP,
      );
      zoom.value = interpolate(
        scale,
        [-1, 0, 1],
        [minZoom, currZoom, maxZoom],
        Extrapolate.CLAMP,
      );
      runOnJS(setZoom)(zoom.value);
    });

  function toggleCamera() {
    setType(type =>
      type === CameraType.back ? CameraType.front : CameraType.back,
    );
  }

  const tapFlipGesture = Gesture.Tap()
    .numberOfTaps(2)
    .onStart(() => {
      runOnJS(toggleCamera)();
    });

  return (
    <View style={styles.container}>
      {isPreview && renderCancelPreviewButton()}
      <GestureDetector
        gesture={Gesture.Simultaneous(pinchZoomGesture, tapFlipGesture)}>
        <View style={StyleSheet.absoluteFill}>
          <Camera
            ref={cameraRef}
            style={styles.camera}
            onCameraReady={onCameraReady}
            zoom={zoomValue}
            type={type}
          />
          {!isPreview ? (
            <TouchableOpacity
              onPress={() => {
                takePicture();
              }}>
              <View style={[styles.circle, styles.bottomButton]} />
            </TouchableOpacity>
          ) : (
            <TouchableOpacity
              onPress={() => {
                keepPicture();
              }}>
              <View style={[styles.bottomButton, styles.rectangle]}>
                <Text style={{fontSize: 15, color: 'white'}}>
                  Use Picture for Card
                </Text>
              </View>
            </TouchableOpacity>
          )}
        </View>
      </GestureDetector>
      <View style={styles.preview} />
    </View>
  );
};

const WINDOW_HEIGHT = Dimensions.get('window').height;
const closeButtonSize = Math.floor(WINDOW_HEIGHT * 0.052);

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
  },
  camera: {
    flex: 1,
  },
  bottomButton: {
    position: 'absolute',
    alignSelf: 'center',
    bottom: 32,
  },
  rectangle: {
    backgroundColor: 'transparent',
    padding: 24,
    borderRadius: 4,
    borderWidth: 3,
    borderColor: 'whitesmoke',
  },
  circle: {
    aspectRatio: '1/1',
    borderRadius: 36,
    width: 72,
    color: 'white',
    borderWidth: 4,
    borderColor: 'whitesmoke',
  },
  buttonContainer: {
    flex: 1,
    flexDirection: 'row',
    backgroundColor: 'transparent',
    margin: 64,
  },
  button: {
    flex: 1,
    width: '100%',
    height: '100%',
  },
  text: {
    fontSize: 24,
    fontWeight: 'bold',
    color: 'white',
  },
  closeButton: {
    position: 'absolute',
    top: 35,
    left: 15,
    height: closeButtonSize,
    width: closeButtonSize,
    borderRadius: Math.floor(closeButtonSize / 2),
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#c4c5c4',
    opacity: 0.7,
    zIndex: 2,
  },

  closeCross: {
    width: '68%',
    height: 1,
    backgroundColor: 'black',
  },
  preview: {
    alignSelf: 'center',
    borderRadius: 25 / 1.4,
    width: '80%',
    aspectRatio: 1 / 1.4,
    backgroundColor: 'transparent',
    pointerEvents: 'none',
    shadowColor: '#000',
    shadowRadius: 10,
    elevation: 24,
    shadowOpacity: 0.5,
    shadowOffset: {width: 0, height: 0},
    borderWidth: 3,
    borderStyle: 'dashed',
    borderColor: 'white',
  },
});

export default CameraScreen;
