import React from 'react';
import {View, Text, StyleSheet, SafeAreaView, ScrollView} from 'react-native';
import {CardStatistics} from './Types';

import SvgAttLvl1 from './assets/AttLvl1';
import SvgAttLvl10 from './assets/AttLvl10';
import SvgAttLvl20 from './assets/AttLvl20';
import SvgAttLvl30 from './assets/AttLvl30';
import SvgAttLvl40 from './assets/AttLvl40';
import SvgAttLvl50 from './assets/AttLvl50';
import SvgAttLvl70 from './assets/AttLvl70';
import SvgAttLvl90 from './assets/AttLvl90';

import SL1 from './assets/StrLvl1';
import SL10 from './assets/StrLvl10';
import SL30 from './assets/StrLvl30';
import SL70 from './assets/StrLvl70';
import SL90 from './assets/StrLvl90';

import DL1 from './assets/DefLvl1';
import DL10 from './assets/DefLvl10';
import DL20 from './assets/DefLvl20';
import DL30 from './assets/DefLvl30';
import DL50 from './assets/DefLvl50';

import HL1 from './assets/HpLvl1';
import HL99 from './assets/HpLvl99';

const getStrengthIcon = (lvl: number) => {
  if (90 <= lvl) {
    return <SL90 />;
  } else if (70 <= lvl) {
    return <SL70 />;
  } else if (30 <= lvl) {
    return <SL30 />;
  } else if (10 <= lvl) {
    return <SL10 />;
  }

  return <SL1 />;
};

const getAttackIcon = (lvl: number) => {
  if (90 <= lvl) {
    return <SvgAttLvl90 />;
  } else if (70 <= lvl) {
    return <SvgAttLvl70 />;
  } else if (50 <= lvl) {
    return <SvgAttLvl50 />;
  } else if (40 <= lvl) {
    return <SvgAttLvl40 />;
  } else if (30 <= lvl) {
    return <SvgAttLvl30 />;
  } else if (20 <= lvl) {
    return <SvgAttLvl20 />;
  } else if (10 <= lvl) {
    return <SvgAttLvl10 />;
  }

  return <SvgAttLvl1 />;
};

const getDefenseIcon = (lvl: number) => {
  if (50 <= lvl) {
    return <DL50 />;
  } else if (30 <= lvl) {
    return <DL30 />;
  } else if (20 <= lvl) {
    return <DL20 />;
  } else if (10 <= lvl) {
    return <DL10 />;
  }
  return <DL1 />;
};

const getHPIcon = (lvl: number) => {
  if (lvl === 99) {
    return <HL99 />;
  }

  return <HL1 />;
};

const CardDetails = (props: CardStatistics) => {
  const {cardType, hitpoints, attack, strength, defense} = props;

  return (
    <View style={styles.container}>
      <View style={styles.statRow}>
        <View>
          <View style={styles.svgWrapper}>{getHPIcon(hitpoints)}</View>
        </View>
        <Text style={styles.text}>{hitpoints}</Text>
      </View>
      <View style={styles.statRow}>
        <View>
          <View style={styles.svgWrapper}>{getAttackIcon(attack)}</View>
        </View>
        <Text style={styles.text}>{attack}</Text>
      </View>
      <View style={styles.statRow}>
        <View>
          <View style={styles.svgWrapper}>{getStrengthIcon(strength)}</View>
        </View>
        <Text style={styles.text}>{strength}</Text>
      </View>
      <View style={styles.statRow}>
        <View>
          <View style={styles.svgWrapper}>{getDefenseIcon(defense)}</View>
        </View>
        <Text style={styles.text}>{defense}</Text>
      </View>
    </View>
  );
};

export default CardDetails;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
    width: '100%',
    alignItems: 'center',
    justifyContent: 'center',
  },
  statRow: {
    flex: 1,
    display: 'flex',
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  svgWrapper: {
    alignItems: 'flex-start',
    flex: 1,
    aspectRatio: 1,
    height: '100%',
  },

  text: {
    fontSize: 18,
    fontWeight: '600',
    paddingRight: 20,
    marginRight: 10,
  },
});
