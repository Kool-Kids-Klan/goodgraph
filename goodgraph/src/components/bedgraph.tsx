import { Data } from 'plotly.js';
import Plot from 'react-plotly.js';

export class Graph {
  graphType: String;
  x: Array<number>;
  y: Array<number>;

  constructor(graphType: String, x: Array<number>, y: Array<number>) {
    this.graphType = graphType;
    this.x = x;
    this.y = y;
  }
}

export class PlotData {
  graphs: Array<Graph>;

  constructor(graphs: Array<Graph>) {
    this.graphs = graphs;
  }
}

function Bedgraph(props: {bedgraphData: PlotData}) {
  let graphsData: Data[] = [];
  props.bedgraphData.graphs.forEach((graph) => (graphsData.push({
      x: graph.x,
      y: graph.y,
      type: graph.graphType,  // figure out how to deal with incorrect type
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
