import { Data } from 'plotly.js';
import Plot from 'react-plotly.js';

export class Graph {
  x: Array<number>;
  y: Array<number>;

  constructor(x: Array<number>, y: Array<number>) {
    this.x = x;
    this.y = y;
  }
}

export class PlotData {
  graphs: Map<String, Graph>;

  constructor(graphs: Object) {
    this.graphs = new Map(Object.entries(graphs));
  }
}

function Bedgraph(props: {bedgraphData: PlotData}) {
  let graphsData: Data[] = [];
  props.bedgraphData.graphs.forEach((graphData, graphType) => (graphsData.push({
      x: graphData.x,
      y: graphData.y,
      type: graphType,  // figure out how to deal with incorrect type
      mode: "lines+markers",
  })))
  return (
    <div>
      {
        <Plot
          data={graphsData}
          layout={ {width: 1000, height: 500, title: {text: "A Fancy Plot"}} }
        />
      }
    </div>
  )
}

export default Bedgraph;
