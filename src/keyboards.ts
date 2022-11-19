interface Keyboard {
  name: string;
  columns: number;
  rows: number;
  layout: Key[];
}

interface Key {
  positionColumn: number;
  spanColumn: number;
  positionRow: number;
  spanRow: number;
  /**
   *  Order :
   *
   * | First element | Second element |
   *
   * | Third element | Fourth element |
   *
   * https://norme-azerty.fr/en/
   *
   */
  label: [NullableString, NullableString, NullableString, NullableString];
}

type NullableString = string | null;

export const azerty: Keyboard = {
  name: "Azerty",
  columns: 14,
  rows: 6,
  layout: [
    {
      label: [null, null, "²", null],
      positionColumn: 1,
      positionRow: 1,
      spanColumn: 1,
      spanRow: 1,
    },
  ],
};
console.log(azerty);
