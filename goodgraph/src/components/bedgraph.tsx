import Plot from 'react-plotly.js';

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
      {
        <Plot
          data={[
            {
              x: props.bedgraphData.x,
              y: props.bedgraphData.y,
              type: "bar",
              mode: "lines+markers",
              marker: {color: "red"},
            },
          ]}
          layout={ {width: 1000, height: 500, title: {text: "A Fancy Plot"}} }
        />
      }
    </div>
  )
}

export default Bedgraph;
