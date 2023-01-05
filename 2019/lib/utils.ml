let file_lines file =
  let rec get_file_lines lines =
    try
      let line = input_line file in
      get_file_lines (line :: lines)
    with _ -> lines
  in
  get_file_lines []
