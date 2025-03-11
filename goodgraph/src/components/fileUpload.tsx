import { ChangeEvent, useState } from 'react';
import { PlotData } from './bedgraph';
import { Dispatch, SetStateAction } from 'react';

class PlotResponse {
  graphs: Object;

  constructor(graphs: Object) {
    this.graphs = graphs;
  }
}

function FileUploadSingle(props: { setBedgraphData: Dispatch<SetStateAction<PlotData>> }) {
  const [file, setFile] = useState<File>();

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  const handleUploadClick = () => {
    if (!file) {
      return;
    }

    let formData = new FormData();
    formData.append('file', file);

    fetch('http://localhost:8080/files', {
      method: 'POST',
      body: formData,
    })
      .then((res) => res.json())
      .then((data: PlotResponse) => {
        console.log(data);
        props.setBedgraphData(new PlotData(data.graphs));
      })
      .catch((err) => console.error(err));
  };

  return (
    <div>
      <input type="file" onChange={handleFileChange} />

      <div>{file && `${file.name} - ${file.type}`}</div>

      <button onClick={handleUploadClick}>Upload</button>
    </div>
  );
}

export default FileUploadSingle;
