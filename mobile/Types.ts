export type CardStatistics = {
  name: string;
  cardType: CardType;
  hitpoints: number;
  attack: number;
  strength: number;
  defense: number;

}

export const cardTypes: CardType[] = ['Fire', 'Air', 'Earth', 'Wind', 'Omni'];

export type CardType = 'Fire' | 'Air' | 'Earth' | 'Wind' | 'Omni';

