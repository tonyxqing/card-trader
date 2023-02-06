import { Camera, CameraType } from 'expo-camera';
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
import { MaterialTopTabScreenProps } from '@react-navigation/material-top-tabs';
import { RootTabParamList } from '../routing';
import * as FileSystem from 'expo-file-system'
import { imageDir } from './CardScreen'
import {CardStatistics, cardTypes} from '../Types'
import AsyncStorage from '@react-native-async-storage/async-storage';
import { newString } from './CardScreen'
type CameraScreenProps = MaterialTopTabScreenProps<
  RootTabParamList,
  'Camera'
>;

const CameraScreen = ({ navigation, route }: CameraScreenProps) => {
  const [type, setType] = React.useState<CameraType>(CameraType.back);
  const [permission, requestPermission] = Camera.useCameraPermissions();
  const [isPreview, setIsPreview] = React.useState(false);
  const [isCameraReady, setIsCameraReady] = React.useState(false);
  const cameraRef = React.useRef<any>();
  const onCameraReady = () => {
    setIsCameraReady(true);
  };
  const cancelPreview = async () => {
    await cameraRef.current.resumePreview();
    setIsPreview(false);
  };
  const renderCancelPreviewButton = () => (
    <TouchableOpacity onPress={cancelPreview} style={styles.closeButton}>
      <View style={[styles.closeCross, { transform: [{ rotate: '45deg' }] }]} />
      <View
        style={[styles.closeCross, { transform: [{ rotate: '-45deg' }] }]}
      />
    </TouchableOpacity>
  );

  const renderTakePictureButton = () => (
    <View style={styles.buttonContainer}>
      <TouchableOpacity
        style={styles.button}
        onPress={takePicture}
        disabled={!isCameraReady}>
        <Text style={styles.text}>Take Picture</Text>
      </TouchableOpacity>
         <TouchableOpacity
        style={styles.button}
        onPress={toggleCameraType}
        disabled={!isCameraReady}>
        <Text style={styles.text}>Flip Picture</Text>
      </TouchableOpacity>
    </View>
  );
  const takePicture = async () => {
    if (cameraRef.current) {
      const options = { quality: 0.5, base64: true, skipProcessing: true, isImageMirror: true};
      const data = await cameraRef.current.takePictureAsync(options);
      const source = data.uri;
      if (source) {
        const date = new Date();
        const fileName = date.toJSON().replace(/[-:TZ.]/gm, '');
        await cameraRef.current.pausePreview();
        setIsPreview(true);
        await FileSystem.moveAsync({ from: source, to: imageDir + fileName})
        const stats: CardStatistics = {
          name: newString(),
          cardType: cardTypes[Math.floor(Math.random() * cardTypes.length)],
          hitpoints: Math.floor(Math.random() * 100),
          attack: Math.floor(Math.random() * 100),
          strength: Math.floor(Math.random() * 100),
          defense: Math.floor(Math.random() * 100), 
        };
      const statString = JSON.stringify(stats)

      await AsyncStorage.setItem(fileName, statString)
        navigation.jumpTo('Card', {picture: fileName})
      }
    }
  };


  if (!permission) {
    // Camera permissions are still loading
    return <View />;
  }


  if (!permission.granted) {
    // Camera permissions are not granted yet
    return (
      <View style={styles.container}>
        <Text style={{ textAlign: 'center' }}>
          We need your permission to show the camera
        </Text>
        <Button onPress={requestPermission} title="grant permission" />
      </View>
    );
  }

  function toggleCameraType() {
    setType((current: CameraType) =>
      current === CameraType.back ? CameraType.front : CameraType.back
    );
  }
  // ### accompanying code
  // <TouchableOpacity style={styles.button} onPress={toggleCameraType}>
  //   <Text style={styles.text}>Flip Camera</Text>
  // </TouchableOpacity>
  // }

  // const textureDims =
  //   Platform.OS === 'ios'
  //     ? {
  //         height: 1920,
  //         width: 1080,
  //       }
  //     : {
  //         height: 1200,
  //         width: 1600,
  //       };

  return (
    <View style={styles.container}>
      {isPreview && renderCancelPreviewButton()}
      <Camera
        ref={cameraRef}
        style={styles.camera}
        type={type}
        onCameraReady={onCameraReady}>
        {!isPreview && renderTakePictureButton()}
      </Camera>
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
});

export default CameraScreen;
