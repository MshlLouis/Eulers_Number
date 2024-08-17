import java.io.*;
import java.math.BigDecimal;
import java.math.RoundingMode;
import java.nio.charset.StandardCharsets;

public class calculateNumber {

    public static void main(String[] args) throws IOException {

        FileOutputStream fos = new FileOutputStream("outResult.txt",true);
        OutputStreamWriter osw = new OutputStreamWriter(fos, StandardCharsets.UTF_8);
        BufferedWriter bw = new BufferedWriter(osw);

        int precision = 100_000;

        String numeratorString = readFile("numerator.txt");
        System.out.println("Read File 1");
        String denominatorString = readFile("denominator.txt");
        System.out.println("Read File 2");

        BigDecimal result;
        BigDecimal numerator = new BigDecimal(numeratorString);
        BigDecimal denominator = new BigDecimal(denominatorString);

        result = new BigDecimal(String.valueOf(numerator)).divide(new BigDecimal(String.valueOf(denominator)), precision, RoundingMode.CEILING);

        int data_split_count = result.toString().length() / 100;

        for(int i = 0; i< data_split_count; i++) {

            String out = (result.toString().substring(i * 100,(i + 1) * 100 ));
            bw.write(out +"\n");
            System.out.println(out);
        }

    }

    private static String readFile(String name) throws IOException {

        BufferedReader br = new BufferedReader(new FileReader(name));
        String line = "";
        StringBuilder full = new StringBuilder();

        while ((line = br.readLine()) != null)
        {
            full.append(line);
        }

        return full.toString();
    }
}
