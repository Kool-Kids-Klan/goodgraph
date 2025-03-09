
export class BedgraphDataPoints {
  x: Array<number>;
  y: Array<number>;

  constructor(x: Array<number>, y: Array<number>) {
    this.x = x;
    this.y = y;
  }
}

function Bedgraph(props: {bedgraphData: BedgraphDataPoints}) {
  return (
    <div>
      <ul>
      {
        props.bedgraphData.x.map(function(_x, i) {
          return (
            <li>{_x} - {props.bedgraphData.y[i]}</li>
          )
        })
      }
      </ul>
    </div>
  )
}

export default Bedgraph;
