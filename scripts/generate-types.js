#!/usr/bin/env node
/**
 * Generate TypeScript types from AsyncAPI specification using Modelina
 * 
 * Usage: node generate-types.js <asyncapi-spec.json> <output-dir>
 */

import { TypeScriptGenerator } from '@asyncapi/modelina';
import fs from 'fs';
import path from 'path';

async function generateTypes(specPath, outputDir) {
  try {
    // Read the AsyncAPI specification
    const specContent = fs.readFileSync(specPath, 'utf8');
    const asyncapiDoc = JSON.parse(specContent);

    // Create TypeScript generator
    const generator = new TypeScriptGenerator({
      // Generate models with proper naming
      modelType: 'interface',
      // Include JSDoc comments
      includeJSDoc: true,
      // Preserve original property names
      preservePropertyNames: true,
    });

    // Generate TypeScript models
    console.log('Generating TypeScript types from AsyncAPI specification...');
    const models = await generator.generate(asyncapiDoc);

    if (models.length === 0) {
      console.warn('Warning: No models were generated. This might indicate an issue with the AsyncAPI specification.');
      return;
    }

    // Ensure output directory exists
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    // Write each model to a file
    let writtenCount = 0;
    for (const model of models) {
      // Use the model name as the filename
      const fileName = `${model.modelName}.ts`;
      const filePath = path.join(outputDir, fileName);
      
      // Write the model content
      fs.writeFileSync(filePath, model.result, 'utf8');
      writtenCount++;
      
      console.log(`  ✓ Generated: ${fileName}`);
    }

    // Generate an index file that exports all types
    const indexContent = models
      .map(model => `export * from './${model.modelName}';`)
      .join('\n');

    const indexPath = path.join(outputDir, 'index.ts');
    fs.writeFileSync(indexPath, indexContent, 'utf8');
    console.log(`  ✓ Generated: index.ts`);

    console.log(`\nSuccessfully generated ${writtenCount} TypeScript type definition(s) in ${outputDir}`);
  } catch (error) {
    console.error('Error generating TypeScript types:', error);
    process.exit(1);
  }
}

// Parse command line arguments
const args = process.argv.slice(2);

if (args.length < 2) {
  console.error('Usage: node generate-types.js <asyncapi-spec.json> <output-dir>');
  process.exit(1);
}

const [specPath, outputDir] = args;

// Validate spec file exists
if (!fs.existsSync(specPath)) {
  console.error(`Error: AsyncAPI specification file not found: ${specPath}`);
  process.exit(1);
}

// Generate types
generateTypes(specPath, outputDir).catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});

