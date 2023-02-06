import React from 'react'
import { View, Image, StyleSheet,TouchableOpacity, SafeAreaView, ScrollView } from 'react-native'
import * as FileSystem from 'expo-file-system'
import { imageDir } from './CardScreen'
import { MaterialTopTabScreenProps } from '@react-navigation/material-top-tabs'
import { RootTabParamList } from '../routing'
import { LinearGradient } from 'expo-linear-gradient';

type CollectionScreenProps = MaterialTopTabScreenProps<RootTabParamList, 'Collection'>;
const CollectionScreen = ({navigation, route}: CollectionScreenProps) => {
  const [cardCollection, setCardCollection] = React.useState<string[]>([])

  React.useEffect(() => {
    const loadCards = async () => {
      const files = await FileSystem.readDirectoryAsync(imageDir);
      setCardCollection(files);
    }
    const unsubscribe = navigation.addListener('focus', () => {
      // The screen is focused
      // Call any action
      loadCards();
    });

    // Return the function to unsubscribe from the event so it gets removed on unmount
    return unsubscribe;
  }, [navigation]);
  const displayCards = (uris: string[]) => {
    return (
        uris.map((uri) => (
        <TouchableOpacity disabled={uri === route.params?.picture} onPress={() => {
          navigation.jumpTo('Card', {picture: uri})
          // FileSystem.deleteAsync(imageDir + uri)
        }}>
          <Image style={styles.smallCard} source={{ uri: imageDir + uri }} resizeMode="cover"/>
        </TouchableOpacity>
        ))
    );
  };
  return <View style={{height: '100%'}}>
        <LinearGradient
          colors={['#ffc3a0','#ffafbd']} 
          start={{x: 0.2, y:0}}
          end={{x: 0.8, y:0.0}}
          style={styles.background}
        />
        <SafeAreaView>
      <ScrollView style={{height: '100%'}}>
      <View style={styles.container}>
      {cardCollection && displayCards(cardCollection)}
  </View></ScrollView></SafeAreaView></View>
}

export default CollectionScreen;

const styles = StyleSheet.create({
    container: {
      margin: 25,
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap'
      

    },
      background: {
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    height: '100%',
    backfaceVisibility: 'hidden',
    zIndex: -150
  },
    smallCard: {
      borderRadius: 25 / 1.618,
      aspectRatio: 1 / 1.618,
      height: 100,
      borderWidth: 0.2,
      margin: 5,
      padding: 5
  },
})